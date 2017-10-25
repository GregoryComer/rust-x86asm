use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 206], OperandSize::Dword)
}

fn pmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1219385287, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 156, 215, 199, 87, 174, 72], OperandSize::Dword)
}

fn pmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 233], OperandSize::Qword)
}

fn pmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(MM4)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Two, 89136498, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 4, 164, 66, 114, 29, 80, 5], OperandSize::Qword)
}

fn pmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 199], OperandSize::Dword)
}

fn pmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(EDX, 695305563, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 130, 91, 133, 113, 41], OperandSize::Dword)
}

fn pmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 196], OperandSize::Qword)
}

fn pmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 4, 31], OperandSize::Qword)
}

