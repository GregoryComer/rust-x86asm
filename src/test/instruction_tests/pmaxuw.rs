use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaxuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 235], OperandSize::Dword)
}

fn pmaxuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1300239391, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 52, 253, 31, 20, 128, 77], OperandSize::Dword)
}

fn pmaxuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 247], OperandSize::Qword)
}

fn pmaxuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMAXUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 765474649, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 62, 12, 197, 89, 55, 160, 45], OperandSize::Qword)
}

