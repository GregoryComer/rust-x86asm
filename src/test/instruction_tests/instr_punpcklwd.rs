use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn punpcklwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 225], OperandSize::Dword)
}

#[test]
fn punpcklwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 31], OperandSize::Dword)
}

#[test]
fn punpcklwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 229], OperandSize::Qword)
}

#[test]
fn punpcklwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1081578884, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 97, 164, 87, 132, 149, 119, 64], OperandSize::Qword)
}

#[test]
fn punpcklwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 246], OperandSize::Dword)
}

#[test]
fn punpcklwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1322643763, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 172, 217, 51, 241, 213, 78], OperandSize::Dword)
}

#[test]
fn punpcklwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 195], OperandSize::Qword)
}

#[test]
fn punpcklwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PUNPCKLWD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 97, 44, 203], OperandSize::Qword)
}

