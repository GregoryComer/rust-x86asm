use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminsb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 56, 231], OperandSize::Dword)
}

#[test]
fn vpminsb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(ECX, 1346568933, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 56, 129, 229, 2, 67, 80], OperandSize::Dword)
}

#[test]
fn vpminsb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 56, 199], OperandSize::Qword)
}

#[test]
fn vpminsb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 56, 7], OperandSize::Qword)
}

#[test]
fn vpminsb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 56, 246], OperandSize::Dword)
}

#[test]
fn vpminsb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 56, 54], OperandSize::Dword)
}

#[test]
fn vpminsb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 56, 253], OperandSize::Qword)
}

#[test]
fn vpminsb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 56, 25], OperandSize::Qword)
}

#[test]
fn vpminsb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 69, 140, 56, 220], OperandSize::Dword)
}

#[test]
fn vpminsb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 101, 139, 56, 9], OperandSize::Dword)
}

#[test]
fn vpminsb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 178, 109, 141, 56, 219], OperandSize::Qword)
}

#[test]
fn vpminsb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 484119499, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 37, 138, 56, 12, 117, 203, 19, 219, 28], OperandSize::Qword)
}

#[test]
fn vpminsb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 169, 56, 195], OperandSize::Dword)
}

#[test]
fn vpminsb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 56, 34], OperandSize::Dword)
}

#[test]
fn vpminsb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM11)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 37, 172, 56, 249], OperandSize::Qword)
}

#[test]
fn vpminsb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 69, 169, 56, 23], OperandSize::Qword)
}

#[test]
fn vpminsb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 56, 215], OperandSize::Dword)
}

#[test]
fn vpminsb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Two, 1498994240, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 56, 164, 88, 64, 214, 88, 89], OperandSize::Dword)
}

#[test]
fn vpminsb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 101, 199, 56, 250], OperandSize::Qword)
}

#[test]
fn vpminsb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINSB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1544995600, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 117, 204, 56, 132, 201, 16, 195, 22, 92], OperandSize::Qword)
}

