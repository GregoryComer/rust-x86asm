use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 200], OperandSize::Dword)
}

fn phsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 8], OperandSize::Dword)
}

fn phsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 212], OperandSize::Qword)
}

fn phsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 7, 20, 135], OperandSize::Qword)
}

fn phsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 250], OperandSize::Dword)
}

fn phsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 1081184490, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 183, 234, 144, 113, 64], OperandSize::Dword)
}

fn phsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 225], OperandSize::Qword)
}

fn phsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 7, 39], OperandSize::Qword)
}

