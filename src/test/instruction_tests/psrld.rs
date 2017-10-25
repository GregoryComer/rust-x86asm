use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psrld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Literal8(61)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 215, 61], OperandSize::Dword)
}

fn psrld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM5)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 114, 213, 1], OperandSize::Qword)
}

fn psrld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 210, 34], OperandSize::Dword)
}

fn psrld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM7)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 114, 215, 123], OperandSize::Qword)
}

fn psrld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 249], OperandSize::Dword)
}

fn psrld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 12, 130], OperandSize::Dword)
}

fn psrld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 240], OperandSize::Qword)
}

fn psrld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1492429873, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 210, 156, 186, 49, 172, 244, 88], OperandSize::Qword)
}

fn psrld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 239], OperandSize::Dword)
}

fn psrld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 200978637, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 20, 205, 205, 176, 250, 11], OperandSize::Dword)
}

fn psrld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 239], OperandSize::Qword)
}

fn psrld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 422413994, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 210, 28, 117, 170, 134, 45, 25], OperandSize::Qword)
}

