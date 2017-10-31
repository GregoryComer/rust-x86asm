use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrldq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLDQ, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 218, 127], OperandSize::Dword)
}

#[test]
fn psrldq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLDQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(40)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 221, 40], OperandSize::Qword)
}

