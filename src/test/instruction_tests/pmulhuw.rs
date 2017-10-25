use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmulhuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 217], OperandSize::Dword)
}

fn pmulhuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 382823937, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 60, 205, 1, 110, 209, 22], OperandSize::Dword)
}

fn pmulhuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 239], OperandSize::Qword)
}

fn pmulhuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1653512888, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 228, 12, 213, 184, 154, 142, 98], OperandSize::Qword)
}

fn pmulhuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 213], OperandSize::Dword)
}

fn pmulhuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 27], OperandSize::Dword)
}

fn pmulhuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 236], OperandSize::Qword)
}

fn pmulhuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHUW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RSI, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 228, 60, 222], OperandSize::Qword)
}

