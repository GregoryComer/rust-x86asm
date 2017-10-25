use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn palignr_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM0)), operand2: Some(Direct(MM7)), operand3: Some(Literal8(94)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 199, 94], OperandSize::Dword)
}

fn palignr_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM5)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 895471957, Some(OperandSize::Qword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 44, 77, 85, 209, 95, 53, 43], OperandSize::Dword)
}

fn palignr_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 225, 36], OperandSize::Qword)
}

fn palignr_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Qword), None)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 58, 15, 20, 210, 38], OperandSize::Qword)
}

fn palignr_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 255, 83], OperandSize::Dword)
}

fn palignr_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Four, 444385484, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 132, 146, 204, 200, 124, 26, 46], OperandSize::Dword)
}

fn palignr_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 209, 31], OperandSize::Qword)
}

fn palignr_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PALIGNR, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(85)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 58, 15, 1, 85], OperandSize::Qword)
}

