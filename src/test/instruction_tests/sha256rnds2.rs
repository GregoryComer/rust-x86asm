use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn sha256rnds2_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 205], OperandSize::Dword)
}

fn sha256rnds2_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 10], OperandSize::Dword)
}

fn sha256rnds2_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 223], OperandSize::Qword)
}

fn sha256rnds2_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHA256RNDS2, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 203, 48], OperandSize::Qword)
}

