use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 248], OperandSize::Dword)
}

fn pmaxsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 580965369, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 177, 249, 211, 160, 34], OperandSize::Dword)
}

fn pmaxsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 238], OperandSize::Qword)
}

fn pmaxsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXSD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Two, 717443148, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 61, 132, 90, 76, 80, 195, 42], OperandSize::Qword)
}

