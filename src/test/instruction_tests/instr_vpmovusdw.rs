use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 19, 212], OperandSize::Dword)
}

#[test]
fn vpmovusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(EDX, 226966864, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 19, 138, 80, 61, 135, 13], OperandSize::Dword)
}

#[test]
fn vpmovusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 126, 143, 19, 227], OperandSize::Qword)
}

#[test]
fn vpmovusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 8, 19, 10], OperandSize::Qword)
}

#[test]
fn vpmovusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 19, 214], OperandSize::Dword)
}

#[test]
fn vpmovusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 533278974, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 180, 250, 254, 48, 201, 31], OperandSize::Dword)
}

#[test]
fn vpmovusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 126, 169, 19, 193], OperandSize::Qword)
}

#[test]
fn vpmovusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1076381282, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 19, 4, 77, 98, 70, 40, 64], OperandSize::Qword)
}

#[test]
fn vpmovusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 19, 244], OperandSize::Dword)
}

#[test]
fn vpmovusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectDisplaced(ESI, 1346001912, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 150, 248, 91, 58, 80], OperandSize::Dword)
}

#[test]
fn vpmovusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 126, 203, 19, 240], OperandSize::Qword)
}

#[test]
fn vpmovusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSDW, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 122150543, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 19, 36, 221, 143, 222, 71, 7], OperandSize::Qword)
}

