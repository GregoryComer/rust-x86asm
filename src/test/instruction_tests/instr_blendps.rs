use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn blendps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 233, 21], OperandSize::Dword)
}

#[test]
fn blendps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 32, 6], OperandSize::Dword)
}

#[test]
fn blendps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 237, 36], OperandSize::Qword)
}

#[test]
fn blendps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BLENDPS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 970221370, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(125)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 12, 164, 210, 58, 103, 212, 57, 125], OperandSize::Qword)
}

