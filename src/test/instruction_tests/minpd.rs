use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn minpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 201], OperandSize::Dword)
}

fn minpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1159812028, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 20, 141, 188, 83, 33, 69], OperandSize::Dword)
}

fn minpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 213], OperandSize::Qword)
}

fn minpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MINPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Two, 1480174286, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 93, 172, 78, 206, 170, 57, 88], OperandSize::Qword)
}

