use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmovzxwq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 192], OperandSize::Dword)
}

fn pmovzxwq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 1354422998, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 180, 119, 214, 218, 186, 80], OperandSize::Dword)
}

fn pmovzxwq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 238], OperandSize::Qword)
}

fn pmovzxwq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMOVZXWQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 52, 12, 81], OperandSize::Qword)
}

