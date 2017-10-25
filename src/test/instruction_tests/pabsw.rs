use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pabsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 247], OperandSize::Dword)
}

fn pabsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 24], OperandSize::Dword)
}

fn pabsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 215], OperandSize::Qword)
}

fn pabsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1463145109, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 29, 20, 221, 149, 210, 53, 87], OperandSize::Qword)
}

fn pabsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 225], OperandSize::Dword)
}

fn pabsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 60, 126], OperandSize::Dword)
}

fn pabsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 206], OperandSize::Qword)
}

fn pabsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PABSW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 29, 12, 71], OperandSize::Qword)
}

