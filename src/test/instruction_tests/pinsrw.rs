use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn pinsrw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM3)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 223, 40], OperandSize::Dword)
}

fn pinsrw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM5)), operand2: Some(IndirectDisplaced(EDX, 1860255809, Some(OperandSize::Word), None)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 170, 65, 64, 225, 110, 66], OperandSize::Dword)
}

fn pinsrw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM1)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 205, 59], OperandSize::Qword)
}

fn pinsrw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 780133484, Some(OperandSize::Word), None)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 196, 60, 221, 108, 228, 127, 46, 102], OperandSize::Qword)
}

fn pinsrw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(61)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 236, 61], OperandSize::Dword)
}

fn pinsrw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Eight, 1716890987, Some(OperandSize::Word), None)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 132, 219, 107, 173, 85, 102, 16], OperandSize::Dword)
}

fn pinsrw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(13)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 246, 13], OperandSize::Qword)
}

fn pinsrw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PINSRW, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Word), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 196, 28, 147, 82], OperandSize::Qword)
}

