use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn paddsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM7)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 252], OperandSize::Dword)
}

fn paddsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 54], OperandSize::Dword)
}

fn paddsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 227], OperandSize::Qword)
}

fn paddsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 2006940962, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 237, 12, 245, 34, 125, 159, 119], OperandSize::Qword)
}

fn paddsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 215], OperandSize::Dword)
}

fn paddsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(ECX, 1334181480, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 161, 104, 254, 133, 79], OperandSize::Dword)
}

fn paddsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 240], OperandSize::Qword)
}

fn paddsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PADDSW, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 237, 27], OperandSize::Qword)
}

