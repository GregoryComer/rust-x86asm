use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcomiss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 213], OperandSize::Dword)
}

fn vcomiss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1377132114, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 36, 181, 82, 94, 21, 82], OperandSize::Dword)
}

fn vcomiss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 206], OperandSize::Qword)
}

fn vcomiss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 17], OperandSize::Qword)
}

fn vcomiss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 124, 24, 47, 238], OperandSize::Dword)
}

fn vcomiss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 428223822, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 47, 134, 78, 45, 134, 25], OperandSize::Dword)
}

fn vcomiss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 209, 124, 24, 47, 233], OperandSize::Qword)
}

fn vcomiss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCOMISS, operand1: Some(Direct(XMM10)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1305230434, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 120, 47, 20, 181, 98, 60, 204, 77], OperandSize::Qword)
}

