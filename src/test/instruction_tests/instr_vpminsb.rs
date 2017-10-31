use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 56, 248], OperandSize::Dword)
}

#[test]
fn vpminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 35866635, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 56, 44, 221, 11, 72, 35, 2], OperandSize::Dword)
}

#[test]
fn vpminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 56, 194], OperandSize::Qword)
}

#[test]
fn vpminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RAX, Four, 1465339485, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 56, 4, 133, 93, 78, 87, 87], OperandSize::Qword)
}

#[test]
fn vpminsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 56, 228], OperandSize::Dword)
}

#[test]
fn vpminsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 56, 60, 192], OperandSize::Dword)
}

#[test]
fn vpminsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 56, 230], OperandSize::Qword)
}

#[test]
fn vpminsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 56, 60, 66], OperandSize::Qword)
}

#[test]
fn vpminsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 142, 56, 216], OperandSize::Dword)
}

#[test]
fn vpminsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 1616195472, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 56, 187, 144, 47, 85, 96], OperandSize::Dword)
}

#[test]
fn vpminsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM15)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 82, 5, 138, 56, 206], OperandSize::Qword)
}

#[test]
fn vpminsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RAX, 1651239082, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 56, 128, 170, 232, 107, 98], OperandSize::Qword)
}

#[test]
fn vpminsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 170, 56, 247], OperandSize::Dword)
}

#[test]
fn vpminsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 741053578, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 117, 173, 56, 60, 77, 138, 148, 43, 44], OperandSize::Dword)
}

#[test]
fn vpminsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 93, 165, 56, 235], OperandSize::Qword)
}

#[test]
fn vpminsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1052467676, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 37, 175, 56, 60, 245, 220, 97, 187, 62], OperandSize::Qword)
}

#[test]
fn vpminsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 203, 56, 246], OperandSize::Dword)
}

#[test]
fn vpminsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectDisplaced(ECX, 2136489746, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 117, 203, 56, 185, 18, 63, 88, 127], OperandSize::Dword)
}

#[test]
fn vpminsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 69, 201, 56, 247], OperandSize::Qword)
}

#[test]
fn vpminsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 21, 198, 56, 12, 144], OperandSize::Qword)
}

