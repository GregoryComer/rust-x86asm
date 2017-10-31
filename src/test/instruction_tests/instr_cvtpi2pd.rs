use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cvtpi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 197], OperandSize::Word)
}

#[test]
fn cvtpi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(DI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 29], OperandSize::Word)
}

#[test]
fn cvtpi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 220], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 41], OperandSize::Dword)
}

#[test]
fn cvtpi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 241], OperandSize::Qword)
}

#[test]
fn cvtpi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CVTPI2PD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RDI, 1958148177, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 42, 175, 81, 248, 182, 116], OperandSize::Qword)
}

