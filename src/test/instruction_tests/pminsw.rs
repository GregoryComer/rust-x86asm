use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pminsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 227], OperandSize::Dword)
}

fn pminsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(EAX, 599426157, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 144, 109, 132, 186, 35], OperandSize::Dword)
}

fn pminsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 253], OperandSize::Qword)
}

fn pminsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(MM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 234, 31], OperandSize::Qword)
}

fn pminsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 227], OperandSize::Dword)
}

fn pminsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 33], OperandSize::Dword)
}

fn pminsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 199], OperandSize::Qword)
}

fn pminsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMINSW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 234, 24], OperandSize::Qword)
}

