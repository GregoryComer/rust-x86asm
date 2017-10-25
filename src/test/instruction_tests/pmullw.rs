use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmullw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 235], OperandSize::Dword)
}

fn pmullw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 46], OperandSize::Dword)
}

fn pmullw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 214], OperandSize::Qword)
}

fn pmullw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RBX, 720651275, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 213, 147, 11, 68, 244, 42], OperandSize::Qword)
}

fn pmullw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 233], OperandSize::Dword)
}

fn pmullw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 659792396, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 52, 117, 12, 162, 83, 39], OperandSize::Dword)
}

fn pmullw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 253], OperandSize::Qword)
}

fn pmullw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RBX, 1730027122, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 213, 139, 114, 30, 30, 103], OperandSize::Qword)
}

