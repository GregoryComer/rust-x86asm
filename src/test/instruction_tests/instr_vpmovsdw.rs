use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 35, 195], OperandSize::Dword)
}

#[test]
fn vpmovsdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 35, 12, 130], OperandSize::Dword)
}

#[test]
fn vpmovsdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 50, 126, 137, 35, 194], OperandSize::Qword)
}

#[test]
fn vpmovsdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(RBX, Two, 153118752, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 8, 35, 12, 93, 32, 104, 32, 9], OperandSize::Qword)
}

#[test]
fn vpmovsdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 171, 35, 232], OperandSize::Dword)
}

#[test]
fn vpmovsdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 35, 3], OperandSize::Dword)
}

#[test]
fn vpmovsdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(XMM14)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 82, 126, 173, 35, 214], OperandSize::Qword)
}

#[test]
fn vpmovsdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 2031273056, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 35, 44, 253, 96, 196, 18, 121], OperandSize::Qword)
}

#[test]
fn vpmovsdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 126, 203, 35, 222], OperandSize::Dword)
}

#[test]
fn vpmovsdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectDisplaced(ESI, 1858746686, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 35, 174, 62, 57, 202, 110], OperandSize::Dword)
}

#[test]
fn vpmovsdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(Direct(YMM14)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 66, 126, 203, 35, 246], OperandSize::Qword)
}

#[test]
fn vpmovsdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSDW, operand1: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 35, 44, 139], OperandSize::Qword)
}

