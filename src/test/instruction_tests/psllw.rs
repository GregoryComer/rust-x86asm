use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psllw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM0)), operand2: Some(Literal8(26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 240, 26], OperandSize::Dword)
}

fn psllw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM3)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 113, 243, 92], OperandSize::Qword)
}

fn psllw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 247, 18], OperandSize::Dword)
}

fn psllw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM3)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 113, 243, 33], OperandSize::Qword)
}

fn psllw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 229], OperandSize::Dword)
}

fn psllw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1657837980, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 4, 253, 156, 153, 208, 98], OperandSize::Dword)
}

fn psllw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 194], OperandSize::Qword)
}

fn psllw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 346633406, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 241, 60, 205, 190, 52, 169, 20], OperandSize::Qword)
}

fn psllw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 225], OperandSize::Dword)
}

fn psllw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(ECX, 335163511, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 145, 119, 48, 250, 19], OperandSize::Dword)
}

fn psllw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 244], OperandSize::Qword)
}

fn psllw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSLLW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 174039456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 241, 132, 142, 160, 161, 95, 10], OperandSize::Qword)
}

