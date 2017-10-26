use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 56, 249], OperandSize::Dword)
}

#[test]
fn vpminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Eight, 1940219654, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 56, 172, 200, 6, 103, 165, 115], OperandSize::Dword)
}

#[test]
fn vpminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 56, 223], OperandSize::Qword)
}

#[test]
fn vpminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 56, 17], OperandSize::Qword)
}

#[test]
fn vpminsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 56, 226], OperandSize::Dword)
}

#[test]
fn vpminsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 779680465, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 56, 152, 209, 250, 120, 46], OperandSize::Dword)
}

#[test]
fn vpminsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 56, 201], OperandSize::Qword)
}

#[test]
fn vpminsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 624631735, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 56, 145, 183, 31, 59, 37], OperandSize::Qword)
}

#[test]
fn vpminsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 137, 56, 204], OperandSize::Dword)
}

#[test]
fn vpminsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 56, 62], OperandSize::Dword)
}

#[test]
fn vpminsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM11)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 37, 142, 56, 212], OperandSize::Qword)
}

#[test]
fn vpminsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 53, 140, 56, 44, 134], OperandSize::Qword)
}

#[test]
fn vpminsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 56, 198], OperandSize::Dword)
}

#[test]
fn vpminsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 171, 56, 63], OperandSize::Dword)
}

#[test]
fn vpminsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 5, 174, 56, 238], OperandSize::Qword)
}

#[test]
fn vpminsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectDisplaced(RBX, 400244594, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 85, 161, 56, 171, 114, 63, 219, 23], OperandSize::Qword)
}

#[test]
fn vpminsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 205, 56, 224], OperandSize::Dword)
}

#[test]
fn vpminsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 334261292, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 56, 185, 44, 108, 236, 19], OperandSize::Dword)
}

#[test]
fn vpminsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 29, 201, 56, 223], OperandSize::Qword)
}

#[test]
fn vpminsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(RBX, 446199826, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 69, 205, 56, 163, 18, 120, 152, 26], OperandSize::Qword)
}

