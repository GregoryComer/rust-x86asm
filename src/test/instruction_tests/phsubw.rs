use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phsubw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 212], OperandSize::Dword)
}

fn phsubw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1039205912, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 148, 143, 24, 6, 241, 61], OperandSize::Dword)
}

fn phsubw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 230], OperandSize::Qword)
}

fn phsubw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 904227609, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 5, 44, 85, 25, 107, 229, 53], OperandSize::Qword)
}

fn phsubw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 236], OperandSize::Dword)
}

fn phsubw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 41], OperandSize::Dword)
}

fn phsubw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 207], OperandSize::Qword)
}

fn phsubw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectDisplaced(RAX, 1403174160, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 5, 184, 16, 189, 162, 83], OperandSize::Qword)
}

