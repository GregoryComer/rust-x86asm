use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn insertps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 255, 125], OperandSize::Dword)
}

#[test]
fn insertps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 898704802, Some(OperandSize::Dword), None)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 60, 149, 162, 37, 145, 53, 68], OperandSize::Dword)
}

#[test]
fn insertps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 248, 120], OperandSize::Qword)
}

#[test]
fn insertps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::INSERTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Dword), None)), operand3: Some(Literal8(26)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 33, 44, 242, 26], OperandSize::Qword)
}

