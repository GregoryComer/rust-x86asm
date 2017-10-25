use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn phaddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 214], OperandSize::Dword)
}

fn phaddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 57], OperandSize::Dword)
}

fn phaddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 250], OperandSize::Qword)
}

fn phaddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 56, 3, 54], OperandSize::Qword)
}

fn phaddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 232], OperandSize::Dword)
}

fn phaddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1703796804, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 161, 68, 224, 141, 101], OperandSize::Dword)
}

fn phaddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 224], OperandSize::Qword)
}

fn phaddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PHADDSW, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RAX, 1490875986, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 56, 3, 144, 82, 246, 220, 88], OperandSize::Qword)
}

