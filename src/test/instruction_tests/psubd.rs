use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 208], OperandSize::Dword)
}

fn psubd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 44, 183], OperandSize::Dword)
}

fn psubd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 238], OperandSize::Qword)
}

fn psubd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(MM4)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 250, 34], OperandSize::Qword)
}

fn psubd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 209], OperandSize::Dword)
}

fn psubd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EBX, EBX, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 4, 155], OperandSize::Dword)
}

fn psubd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 200], OperandSize::Qword)
}

fn psubd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 437508041, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 250, 20, 221, 201, 215, 19, 26], OperandSize::Qword)
}

