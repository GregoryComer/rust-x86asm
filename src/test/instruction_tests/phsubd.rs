use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phsubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 199], OperandSize::Dword)
}

fn phsubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM7)), operand2: Some(IndirectDisplaced(EBX, 1872283850, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 187, 202, 200, 152, 111], OperandSize::Dword)
}

fn phsubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 207], OperandSize::Qword)
}

fn phsubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 175562948, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 6, 52, 181, 196, 224, 118, 10], OperandSize::Qword)
}

fn phsubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 239], OperandSize::Dword)
}

fn phsubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 638143933, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 52, 189, 189, 77, 9, 38], OperandSize::Dword)
}

fn phsubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 204], OperandSize::Qword)
}

fn phsubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDI, Two, 1847162669, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 6, 132, 127, 45, 119, 25, 110], OperandSize::Qword)
}

