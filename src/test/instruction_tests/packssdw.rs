use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn packssdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 235], OperandSize::Dword)
}

fn packssdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 52, 88], OperandSize::Dword)
}

fn packssdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 232], OperandSize::Qword)
}

fn packssdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 107, 28, 219], OperandSize::Qword)
}

fn packssdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 206], OperandSize::Dword)
}

fn packssdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 1189346879, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 12, 133, 63, 254, 227, 70], OperandSize::Dword)
}

fn packssdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 200], OperandSize::Qword)
}

fn packssdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PACKSSDW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1384891371, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 107, 52, 221, 235, 195, 139, 82], OperandSize::Qword)
}

