use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 226], OperandSize::Dword)
}

fn movss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 215], OperandSize::Qword)
}

fn movss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ECX, 1675344227, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 129, 99, 185, 219, 99], OperandSize::Dword)
}

fn movss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 3], OperandSize::Qword)
}

fn movss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 241], OperandSize::Dword)
}

fn movss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexed(EBX, EDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 12, 187], OperandSize::Dword)
}

fn movss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 16, 219], OperandSize::Qword)
}

fn movss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVSS, operand1: Some(IndirectScaledIndexed(RBX, RBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[243, 15, 17, 12, 155], OperandSize::Qword)
}

