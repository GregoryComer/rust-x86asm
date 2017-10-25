use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubusb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 245], OperandSize::Dword)
}

fn psubusb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM6)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 49], OperandSize::Dword)
}

fn psubusb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 252], OperandSize::Qword)
}

fn psubusb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 216, 44, 255], OperandSize::Qword)
}

fn psubusb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 212], OperandSize::Dword)
}

fn psubusb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(ECX, 398888278, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 177, 86, 141, 198, 23], OperandSize::Dword)
}

fn psubusb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 223], OperandSize::Qword)
}

fn psubusb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1178493912, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 216, 184, 216, 99, 62, 70], OperandSize::Qword)
}

