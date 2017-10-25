use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vphminposuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 223], OperandSize::Dword)
}

fn vphminposuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 2143451290, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 12, 253, 154, 120, 194, 127], OperandSize::Dword)
}

fn vphminposuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 230], OperandSize::Qword)
}

fn vphminposuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHMINPOSUW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(RSI, 1966894965, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 65, 142, 117, 111, 60, 117], OperandSize::Qword)
}

