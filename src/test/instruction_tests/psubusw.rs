use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn psubusw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 246], OperandSize::Dword)
}

fn psubusw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledIndexed(EAX, ESI, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 20, 176], OperandSize::Dword)
}

fn psubusw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 230], OperandSize::Qword)
}

fn psubusw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 217, 50], OperandSize::Qword)
}

fn psubusw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 254], OperandSize::Dword)
}

fn psubusw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 46], OperandSize::Dword)
}

fn psubusw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 242], OperandSize::Qword)
}

fn psubusw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBUSW, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 684456985, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 217, 36, 253, 25, 252, 203, 40], OperandSize::Qword)
}

