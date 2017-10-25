use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn rorx_1() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 231, 23], OperandSize::Dword)
}

fn rorx_2() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(EDX, 2117374934, Some(OperandSize::Dword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 186, 214, 147, 52, 126, 96], OperandSize::Dword)
}

fn rorx_3() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(49)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 255, 49], OperandSize::Qword)
}

fn rorx_4() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(EDX)), operand2: Some(IndirectDisplaced(RDI, 1453939990, Some(OperandSize::Dword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 123, 240, 151, 22, 93, 169, 86, 77], OperandSize::Qword)
}

fn rorx_5() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 251, 54], OperandSize::Qword)
}

fn rorx_6() {
    run_test(&Instruction { mnemonic: Mnemonic::RORX, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1519797236, Some(OperandSize::Qword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 251, 240, 12, 245, 244, 67, 150, 90, 77], OperandSize::Qword)
}

