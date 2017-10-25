use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pcmpgtb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 229], OperandSize::Dword)
}

fn pcmpgtb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 44, 118], OperandSize::Dword)
}

fn pcmpgtb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 205], OperandSize::Qword)
}

fn pcmpgtb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(MM3)), operand2: Some(IndirectDisplaced(RCX, 778391273, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 100, 153, 233, 78, 101, 46], OperandSize::Qword)
}

fn pcmpgtb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 205], OperandSize::Dword)
}

fn pcmpgtb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 1924149397, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 172, 211, 149, 48, 176, 114], OperandSize::Dword)
}

fn pcmpgtb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 235], OperandSize::Qword)
}

fn pcmpgtb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PCMPGTB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(RBX, 1697409226, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 100, 179, 202, 104, 44, 101], OperandSize::Qword)
}

