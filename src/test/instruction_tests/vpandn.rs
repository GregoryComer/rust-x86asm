use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 223, 216], OperandSize::Dword)
}

fn vpandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1803233859, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 223, 60, 221, 67, 42, 123, 107], OperandSize::Dword)
}

fn vpandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 223, 223], OperandSize::Qword)
}

fn vpandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1130558334, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 223, 12, 85, 126, 243, 98, 67], OperandSize::Qword)
}

fn vpandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 223, 208], OperandSize::Dword)
}

fn vpandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 1937934551, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 223, 132, 153, 215, 136, 130, 115], OperandSize::Dword)
}

fn vpandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 223, 208], OperandSize::Qword)
}

fn vpandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 223, 57], OperandSize::Qword)
}

