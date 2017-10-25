use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 206], OperandSize::Dword)
}

fn psubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(ECX, 1052671746, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 169, 2, 127, 190, 62], OperandSize::Dword)
}

fn psubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 195], OperandSize::Qword)
}

fn psubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(RBX, 915636675, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 233, 171, 195, 129, 147, 54], OperandSize::Qword)
}

fn psubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 233], OperandSize::Dword)
}

fn psubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Four, 1715397128, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 60, 149, 8, 226, 62, 102], OperandSize::Dword)
}

fn psubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 236], OperandSize::Qword)
}

fn psubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1344693522, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 233, 20, 149, 18, 101, 38, 80], OperandSize::Qword)
}

