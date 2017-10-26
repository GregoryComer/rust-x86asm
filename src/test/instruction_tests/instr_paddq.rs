use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn paddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 250], OperandSize::Dword)
}

#[test]
fn paddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EDX, 163433007, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 186, 47, 202, 189, 9], OperandSize::Dword)
}

#[test]
fn paddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 202], OperandSize::Qword)
}

#[test]
fn paddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 663949680, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 212, 12, 117, 112, 17, 147, 39], OperandSize::Qword)
}

#[test]
fn paddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 207], OperandSize::Dword)
}

#[test]
fn paddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 804447949, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 185, 205, 230, 242, 47], OperandSize::Dword)
}

#[test]
fn paddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 231], OperandSize::Qword)
}

#[test]
fn paddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Two, 1373428730, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 212, 156, 126, 250, 219, 220, 81], OperandSize::Qword)
}

