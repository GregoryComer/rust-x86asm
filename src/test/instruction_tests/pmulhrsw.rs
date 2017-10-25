use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmulhrsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 238], OperandSize::Dword)
}

fn pmulhrsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1455774709, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 187, 245, 91, 197, 86], OperandSize::Dword)
}

fn pmulhrsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 252], OperandSize::Qword)
}

fn pmulhrsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 11, 38], OperandSize::Qword)
}

fn pmulhrsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 197], OperandSize::Dword)
}

fn pmulhrsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(ECX, 1136121351, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 185, 7, 214, 183, 67], OperandSize::Dword)
}

fn pmulhrsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 206], OperandSize::Qword)
}

fn pmulhrsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHRSW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 11, 3], OperandSize::Qword)
}

