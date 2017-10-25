use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 192], OperandSize::Dword)
}

fn pandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 61159840, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 44, 197, 160, 57, 165, 3], OperandSize::Dword)
}

fn pandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 223], OperandSize::Qword)
}

fn pandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 44580460, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 223, 28, 157, 108, 62, 168, 2], OperandSize::Qword)
}

fn pandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 242], OperandSize::Dword)
}

fn pandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 208512294, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 12, 133, 38, 165, 109, 12], OperandSize::Dword)
}

fn pandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 216], OperandSize::Qword)
}

fn pandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PANDN, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 861725480, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 223, 180, 217, 40, 227, 92, 51], OperandSize::Qword)
}

