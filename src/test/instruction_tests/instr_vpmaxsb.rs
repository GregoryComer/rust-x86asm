use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmaxsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 220], OperandSize::Dword)
}

#[test]
fn vpmaxsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 28, 187], OperandSize::Dword)
}

#[test]
fn vpmaxsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 60, 198], OperandSize::Qword)
}

#[test]
fn vpmaxsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 60, 28, 251], OperandSize::Qword)
}

#[test]
fn vpmaxsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 60, 209], OperandSize::Dword)
}

#[test]
fn vpmaxsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 60, 22], OperandSize::Dword)
}

#[test]
fn vpmaxsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 60, 195], OperandSize::Qword)
}

#[test]
fn vpmaxsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(RBX, 1779750434, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 60, 139, 34, 214, 20, 106], OperandSize::Qword)
}

#[test]
fn vpmaxsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 142, 60, 248], OperandSize::Dword)
}

#[test]
fn vpmaxsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 117, 137, 60, 39], OperandSize::Dword)
}

#[test]
fn vpmaxsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 29, 129, 60, 246], OperandSize::Qword)
}

#[test]
fn vpmaxsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1823710280, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 135, 60, 140, 153, 72, 156, 179, 108], OperandSize::Qword)
}

#[test]
fn vpmaxsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 173, 60, 229], OperandSize::Dword)
}

#[test]
fn vpmaxsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 60, 22], OperandSize::Dword)
}

#[test]
fn vpmaxsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM19)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 162, 45, 172, 60, 223], OperandSize::Qword)
}

#[test]
fn vpmaxsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM9)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 53, 174, 60, 40], OperandSize::Qword)
}

#[test]
fn vpmaxsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 60, 233], OperandSize::Dword)
}

#[test]
fn vpmaxsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 834947450, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 60, 164, 179, 122, 73, 196, 49], OperandSize::Dword)
}

#[test]
fn vpmaxsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 77, 195, 60, 225], OperandSize::Qword)
}

#[test]
fn vpmaxsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMAXSB, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM16)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 984240296, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 125, 194, 60, 164, 153, 168, 80, 170, 58], OperandSize::Qword)
}

