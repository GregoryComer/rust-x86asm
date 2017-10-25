use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movups_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 253], OperandSize::Dword)
}

fn movups_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Four, 1572497834, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 188, 178, 170, 105, 186, 93], OperandSize::Dword)
}

fn movups_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 230], OperandSize::Qword)
}

fn movups_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 44, 218], OperandSize::Qword)
}

fn movups_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 249], OperandSize::Dword)
}

fn movups_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectScaledDisplaced(EDI, Four, 1130431910, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 20, 189, 166, 5, 97, 67], OperandSize::Dword)
}

fn movups_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 16, 201], OperandSize::Qword)
}

fn movups_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVUPS, operand1: Some(IndirectDisplaced(RSI, 376722383, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 17, 158, 207, 83, 116, 22], OperandSize::Qword)
}

