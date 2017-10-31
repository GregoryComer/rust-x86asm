use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 227], OperandSize::Word)
}

#[test]
fn cvtpi2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 149, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 128, 149, 0], OperandSize::Word)
}

#[test]
fn cvtpi2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 197], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1652070751, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 140, 193, 95, 153, 120, 98], OperandSize::Dword)
}

#[test]
fn cvtpi2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 241], OperandSize::Qword)
}

#[test]
fn cvtpi2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 42, 44, 127], OperandSize::Qword)
}

