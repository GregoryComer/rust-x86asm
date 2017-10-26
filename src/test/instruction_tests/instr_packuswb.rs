use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn packuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 208], OperandSize::Dword)
}

#[test]
fn packuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 987200, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 20, 117, 64, 16, 15, 0], OperandSize::Dword)
}

#[test]
fn packuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 213], OperandSize::Qword)
}

#[test]
fn packuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(MM1)), operand2: Some(IndirectDisplaced(RDI, 986416168, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 103, 143, 40, 132, 203, 58], OperandSize::Qword)
}

#[test]
fn packuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 193], OperandSize::Dword)
}

#[test]
fn packuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 871846748, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 36, 197, 92, 83, 247, 51], OperandSize::Dword)
}

#[test]
fn packuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 199], OperandSize::Qword)
}

#[test]
fn packuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1570566552, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 103, 12, 213, 152, 241, 156, 93], OperandSize::Qword)
}

