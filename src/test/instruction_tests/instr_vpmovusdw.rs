use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 19, 208], OperandSize::Dword)
}

#[test]
fn vpmovusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(EBX, Four, 1244384909, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 44, 157, 141, 206, 43, 74], OperandSize::Dword)
}

#[test]
fn vpmovusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 50, 126, 138, 19, 232], OperandSize::Qword)
}

#[test]
fn vpmovusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(RAX, 715741089, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 19, 144, 161, 87, 169, 42], OperandSize::Qword)
}

#[test]
fn vpmovusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 172, 19, 201], OperandSize::Dword)
}

#[test]
fn vpmovusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(EBX, 1395469039, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 139, 239, 42, 45, 83], OperandSize::Dword)
}

#[test]
fn vpmovusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 126, 174, 19, 223], OperandSize::Qword)
}

#[test]
fn vpmovusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 19, 26], OperandSize::Qword)
}

#[test]
fn vpmovusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 205, 19, 240], OperandSize::Dword)
}

#[test]
fn vpmovusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 4, 249], OperandSize::Dword)
}

#[test]
fn vpmovusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM8)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 126, 204, 19, 224], OperandSize::Qword)
}

#[test]
fn vpmovusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 19, 59], OperandSize::Qword)
}

