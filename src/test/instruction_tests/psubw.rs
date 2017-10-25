use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 193], OperandSize::Dword)
}

fn psubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 433183895, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 4, 117, 151, 220, 209, 25], OperandSize::Dword)
}

fn psubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 245], OperandSize::Qword)
}

fn psubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 249, 38], OperandSize::Qword)
}

fn psubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 200], OperandSize::Dword)
}

fn psubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 58], OperandSize::Dword)
}

fn psubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 199], OperandSize::Qword)
}

fn psubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(RCX, RBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 249, 4, 153], OperandSize::Qword)
}

