use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 225], OperandSize::Dword)
}

fn pcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 1409453236, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 4, 181, 180, 140, 2, 84], OperandSize::Dword)
}

fn pcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 253], OperandSize::Qword)
}

fn pcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(MM6)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 102, 52, 135], OperandSize::Qword)
}

fn pcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 251], OperandSize::Dword)
}

fn pcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 6], OperandSize::Dword)
}

fn pcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 203], OperandSize::Qword)
}

fn pcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectDisplaced(RBX, 1543821345, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 102, 155, 33, 216, 4, 92], OperandSize::Qword)
}

