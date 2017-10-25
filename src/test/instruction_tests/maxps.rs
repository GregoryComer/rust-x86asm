use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn maxps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 241], OperandSize::Dword)
}

fn maxps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 46], OperandSize::Dword)
}

fn maxps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 223], OperandSize::Qword)
}

fn maxps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MAXPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 571227307, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 95, 4, 245, 171, 60, 12, 34], OperandSize::Qword)
}

