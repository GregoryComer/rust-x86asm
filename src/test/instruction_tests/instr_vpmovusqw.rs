use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovusqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 20, 227], OperandSize::Dword)
}

#[test]
fn vpmovusqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexed(EDX, EDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 28, 122], OperandSize::Dword)
}

#[test]
fn vpmovusqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 126, 141, 20, 235], OperandSize::Qword)
}

#[test]
fn vpmovusqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 1268928914, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 20, 20, 245, 146, 81, 162, 75], OperandSize::Qword)
}

#[test]
fn vpmovusqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 20, 227], OperandSize::Dword)
}

#[test]
fn vpmovusqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectDisplaced(ECX, 260461027, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 20, 169, 227, 81, 134, 15], OperandSize::Dword)
}

#[test]
fn vpmovusqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM27)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 126, 169, 20, 251], OperandSize::Qword)
}

#[test]
fn vpmovusqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 40, 20, 26], OperandSize::Qword)
}

#[test]
fn vpmovusqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 207, 20, 215], OperandSize::Dword)
}

#[test]
fn vpmovusqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Eight, 75453781, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 132, 251, 85, 85, 127, 4], OperandSize::Dword)
}

#[test]
fn vpmovusqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 20, 251], OperandSize::Qword)
}

#[test]
fn vpmovusqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSQW, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 20, 3], OperandSize::Qword)
}

