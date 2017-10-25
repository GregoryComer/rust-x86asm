use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaddwd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 253], OperandSize::Dword)
}

fn pmaddwd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 51], OperandSize::Dword)
}

fn pmaddwd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 217], OperandSize::Qword)
}

fn pmaddwd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1171639294, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 245, 180, 113, 254, 203, 213, 69], OperandSize::Qword)
}

fn pmaddwd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 238], OperandSize::Dword)
}

fn pmaddwd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 201240965, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 52, 213, 133, 177, 254, 11], OperandSize::Dword)
}

fn pmaddwd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 234], OperandSize::Qword)
}

fn pmaddwd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDWD, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 245, 33], OperandSize::Qword)
}

