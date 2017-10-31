use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sha256rnds2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 242], OperandSize::Dword)
}

#[test]
fn sha256rnds2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 984692920, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 148, 217, 184, 56, 177, 58], OperandSize::Dword)
}

#[test]
fn sha256rnds2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 199], OperandSize::Qword)
}

#[test]
fn sha256rnds2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 20, 150], OperandSize::Qword)
}

