use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 209], OperandSize::Dword)
}

fn psubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 30], OperandSize::Dword)
}

fn psubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 217], OperandSize::Qword)
}

fn psubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 2140908351, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 172, 254, 63, 171, 155, 127], OperandSize::Qword)
}

fn psubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 207], OperandSize::Dword)
}

fn psubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 217692004, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 60, 221, 100, 183, 249, 12], OperandSize::Dword)
}

fn psubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 234], OperandSize::Qword)
}

fn psubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(RDX, RAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 44, 66], OperandSize::Qword)
}

