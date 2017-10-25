use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 203], OperandSize::Dword)
}

fn pmovzxwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 36, 153], OperandSize::Dword)
}

fn pmovzxwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 225], OperandSize::Qword)
}

fn pmovzxwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 51, 58], OperandSize::Qword)
}

