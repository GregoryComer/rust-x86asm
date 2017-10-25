use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 198], OperandSize::Dword)
}

fn pmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 59], OperandSize::Dword)
}

fn pmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 253], OperandSize::Qword)
}

fn pmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledIndexed(RDX, RSI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 229, 4, 178], OperandSize::Qword)
}

fn pmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 206], OperandSize::Dword)
}

fn pmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM1)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 10], OperandSize::Dword)
}

fn pmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 240], OperandSize::Qword)
}

fn pmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMULHW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 229, 60, 82], OperandSize::Qword)
}

